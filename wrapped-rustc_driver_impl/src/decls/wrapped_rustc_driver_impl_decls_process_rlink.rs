use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn process_rlink(sess: &Session, compiler: &interface::Compiler) {
    assert!(sess.opts.unstable_opts.link_only);
    let dcx = sess.dcx();
    if let Input::File(file) = &sess.io.input {
        let rlink_data = fs::read(file)
            .unwrap_or_else(|err| {
                dcx.emit_fatal(RlinkUnableToRead { err });
            });
        let (codegen_results, metadata, outputs) = match CodegenResults::deserialize_rlink(
            sess,
            rlink_data,
        ) {
            Ok((codegen, metadata, outputs)) => (codegen, metadata, outputs),
            Err(err) => {
                match err {
                    CodegenErrors::WrongFileType => dcx.emit_fatal(RLinkWrongFileType),
                    CodegenErrors::EmptyVersionNumber => {
                        dcx.emit_fatal(RLinkEmptyVersionNumber)
                    }
                    CodegenErrors::EncodingVersionMismatch {
                        version_array,
                        rlink_version,
                    } => {
                        dcx.emit_fatal(RLinkEncodingVersionMismatch {
                            version_array,
                            rlink_version,
                        })
                    }
                    CodegenErrors::RustcVersionMismatch { rustc_version } => {
                        dcx.emit_fatal(RLinkRustcVersionMismatch {
                            rustc_version,
                            current_version: sess.cfg_version,
                        })
                    }
                    CodegenErrors::CorruptFile => {
                        dcx.emit_fatal(RlinkCorruptFile { file });
                    }
                };
            }
        };
        compiler.codegen_backend.link(sess, codegen_results, metadata, &outputs);
    } else {
        dcx.emit_fatal(RlinkNotAFile {});
    }
}

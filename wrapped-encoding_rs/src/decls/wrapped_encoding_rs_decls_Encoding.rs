use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// An encoding as defined in the [Encoding Standard][1].
///
/// An _encoding_ defines a mapping from a `u8` sequence to a `char` sequence
/// and, in most cases, vice versa. Each encoding has a name, an output
/// encoding, and one or more labels.
///
/// _Labels_ are ASCII-case-insensitive strings that are used to identify an
/// encoding in formats and protocols. The _name_ of the encoding is the
/// preferred label in the case appropriate for returning from the
/// [`characterSet`][2] property of the `Document` DOM interface.
///
/// The _output encoding_ is the encoding used for form submission and URL
/// parsing on Web pages in the encoding. This is UTF-8 for the replacement,
/// UTF-16LE and UTF-16BE encodings and the encoding itself for other
/// encodings.
///
/// [1]: https://encoding.spec.whatwg.org/
/// [2]: https://dom.spec.whatwg.org/#dom-document-characterset
///
/// # Streaming vs. Non-Streaming
///
/// When you have the entire input in a single buffer, you can use the
/// methods [`decode()`][3], [`decode_with_bom_removal()`][3],
/// [`decode_without_bom_handling()`][5],
/// [`decode_without_bom_handling_and_without_replacement()`][6] and
/// [`encode()`][7]. (These methods are available to Rust callers only and are
/// not available in the C API.) Unlike the rest of the API available to Rust,
/// these methods perform heap allocations. You should the `Decoder` and
/// `Encoder` objects when your input is split into multiple buffers or when
/// you want to control the allocation of the output buffers.
///
/// [3]: #method.decode
/// [4]: #method.decode_with_bom_removal
/// [5]: #method.decode_without_bom_handling
/// [6]: #method.decode_without_bom_handling_and_without_replacement
/// [7]: #method.encode
///
/// # Instances
///
/// All instances of `Encoding` are statically allocated and have the `'static`
/// lifetime. There is precisely one unique `Encoding` instance for each
/// encoding defined in the Encoding Standard.
///
/// To obtain a reference to a particular encoding whose identity you know at
/// compile time, use a `static` that refers to encoding. There is a `static`
/// for each encoding. The `static`s are named in all caps with hyphens
/// replaced with underscores (and in C/C++ have `_ENCODING` appended to the
/// name). For example, if you know at compile time that you will want to
/// decode using the UTF-8 encoding, use the `UTF_8` `static` (`UTF_8_ENCODING`
/// in C/C++).
///
/// Additionally, there are non-reference-typed forms ending with `_INIT` to
/// work around the problem that `static`s of the type `&'static Encoding`
/// cannot be used to initialize items of an array whose type is
/// `[&'static Encoding; N]`.
///
/// If you don't know what encoding you need at compile time and need to
/// dynamically get an encoding by label, use
/// <code>Encoding::<a href="#method.for_label">for_label</a>(<var>label</var>)</code>.
///
/// Instances of `Encoding` can be compared with `==` (in both Rust and in
/// C/C++).
pub struct Encoding {
    name: &'static str,
    variant: VariantEncoding,
}

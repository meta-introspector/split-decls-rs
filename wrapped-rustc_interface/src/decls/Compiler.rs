macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! Compiler {
    () => {
        deps!();
        # [doc = " Represents a compiler session. Note that every `Compiler` contains a"] # [doc = " `Session`, but `Compiler` also contains some things that cannot be in"] # [doc = " `Session`, due to `Session` being in a crate that has many fewer"] # [doc = " dependencies than this crate."] # [doc = ""] # [doc = " Can be used to run `rustc_interface` queries."] # [doc = " Created by passing [`Config`] to [`run_compiler`]."] pub struct Compiler { pub sess : Session , pub codegen_backend : Box < dyn CodegenBackend > , pub (crate) override_queries : Option < fn (& Session , & mut Providers) > , # [doc = " A reference to the current `GlobalCtxt` which we pass on to `GlobalCtxt`."] pub (crate) current_gcx : CurrentGcx , # [doc = " A jobserver reference which we pass on to `GlobalCtxt`."] pub (crate) jobserver_proxy : Arc < Proxy > , }
    };
}

Compiler!()
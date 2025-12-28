macro_rules! DefaultMetadataLoader {
    () => {
        # [doc = " The default metadata loader. This is used by cg_llvm and cg_clif."] # [doc = ""] # [doc = " # Metadata location"] # [doc = ""] # [doc = " <dl>"] # [doc = " <dt>rlib</dt>"] # [doc = " <dd>The metadata can be found in the `lib.rmeta` file inside of the ar archive.</dd>"] # [doc = " <dt>dylib</dt>"] # [doc = " <dd>The metadata can be found in the `.rustc` section of the shared library.</dd>"] # [doc = " </dl>"] # [derive (Debug)] pub (crate) struct DefaultMetadataLoader ;
    };
}

DefaultMetadataLoader!();
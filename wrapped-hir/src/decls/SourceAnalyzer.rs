macro_rules! deps {
    () => {
        BodyOrSig!();
    };
}

macro_rules! SourceAnalyzer {
    () => {
        deps!();
        # [doc = " `SourceAnalyzer` is a convenience wrapper which exposes HIR API in terms of"] # [doc = " original source files. It should not be used inside the HIR itself."] # [derive (Debug)] pub (crate) struct SourceAnalyzer < 'db > { pub (crate) file_id : HirFileId , pub (crate) resolver : Resolver < 'db > , pub (crate) body_or_sig : Option < BodyOrSig < 'db > > , }
    };
}

SourceAnalyzer!()
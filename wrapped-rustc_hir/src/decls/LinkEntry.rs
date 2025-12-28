macro_rules! deps {
    () => {
        PrintAttribute!();
        CfgEntry!();
        PeImportNameType!();
        NativeLibKind!();
    };
}

macro_rules! LinkEntry {
    () => {
        deps!();
        # [derive (Debug , Encodable , Decodable , Clone , HashStable_Generic , PrintAttribute)] pub struct LinkEntry { pub span : Span , pub kind : NativeLibKind , pub name : Symbol , pub cfg : Option < CfgEntry > , pub verbatim : Option < bool > , pub import_name_type : Option < (PeImportNameType , Span) > , }
    };
}

LinkEntry!()
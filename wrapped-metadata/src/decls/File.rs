macro_rules! deps {
    () => {
        Blobs!();
        GenericParam!();
        Strings!();
        Records!();
        ModuleRef!();
        MemberRef!();
        Attribute!();
        AssemblyRef!();
        Constant!();
        TypeRef!();
    };
}

macro_rules! File {
    () => {
        deps!();
        # [doc = " Represents an ECMA-335 file in memory so that it can be built incrementally."] # [derive (Default)] pub struct File { strings : Strings , blobs : Blobs , records : rec :: Records , TypeRef : HashMap < String , HashMap < String , id :: TypeRef > > , AssemblyRef : HashMap < String , id :: AssemblyRef > , ModuleRef : HashMap < String , id :: ModuleRef > , MemberRef : HashMap < rec :: MemberRef , id :: MemberRef > , Constant : BTreeMap < HasConstant , rec :: Constant > , Attribute : BTreeMap < HasAttribute , Vec < rec :: Attribute > > , GenericParam : BTreeMap < TypeOrMethodDef , Vec < rec :: GenericParam > > , }
    };
}

File!()
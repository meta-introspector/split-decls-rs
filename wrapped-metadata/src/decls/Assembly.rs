macro_rules! Assembly {
    () => {
        # [derive (Default)] pub struct Assembly { pub HashAlgId : u32 , pub MajorVersion : u16 , pub MinorVersion : u16 , pub BuildNumber : u16 , pub RevisionNumber : u16 , pub Flags : AssemblyFlags , pub PublicKey : u32 , pub Name : id :: StringId , pub Culture : u32 , }
    };
}

Assembly!();
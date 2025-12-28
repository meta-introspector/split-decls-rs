macro_rules! AssemblyRef {
    () => {
        # [derive (Default)] pub struct AssemblyRef { pub MajorVersion : u16 , pub MinorVersion : u16 , pub BuildNumber : u16 , pub RevisionNumber : u16 , pub Flags : AssemblyFlags , pub PublicKeyOrToken : id :: BlobId , pub Name : id :: StringId , pub Culture : u32 , pub HashValue : u32 , }
    };
}

AssemblyRef!();
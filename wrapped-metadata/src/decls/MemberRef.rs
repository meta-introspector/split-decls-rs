macro_rules! deps {
    () => {
        Signature!();
    };
}

macro_rules! MemberRef {
    () => {
        deps!();
        # [derive (Hash , PartialEq , Eq , Copy , Clone)] pub struct MemberRef { pub Parent : MemberRefParent , pub Name : id :: StringId , pub Signature : id :: BlobId , }
    };
}

MemberRef!()
macro_rules! deps {
    () => {
        FnDecl!();
        FnHeader!();
    };
}

macro_rules! FnSig {
    () => {
        deps!();
        # [doc = " Represents a function's signature in a trait declaration,"] # [doc = " trait implementation, or free function."] # [derive (Clone , Encodable , Decodable , Debug)] pub struct FnSig { pub header : FnHeader , pub decl : Box < FnDecl > , pub span : Span , }
    };
}

FnSig!();
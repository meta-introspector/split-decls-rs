macro_rules! deps {
    () => {
        FnHeader!();
        Walkable!();
        FnRetTy!();
        Param!();
    };
}

macro_rules! FnDecl {
    () => {
        deps!();
        # [doc = " A signature (not the body) of a function declaration."] # [doc = ""] # [doc = " E.g., `fn foo(bar: baz)`."] # [doc = ""] # [doc = " Please note that it's different from `FnHeader` structure"] # [doc = " which contains metadata about function safety, asyncness, constness and ABI."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct FnDecl { pub inputs : ThinVec < Param > , pub output : FnRetTy , }
    };
}

FnDecl!()
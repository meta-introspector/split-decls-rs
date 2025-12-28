macro_rules! deps {
    () => {
        InFileWrapper!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < FileId : Copy , N : AstNode > InFileWrapper < FileId , & N > { pub fn syntax_ref (& self) -> InFileWrapper < FileId , & SyntaxNode > { self . with_value (self . value . syntax ()) } }
    };
}

impl_95!();
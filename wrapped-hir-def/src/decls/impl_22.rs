macro_rules! deps {
    () => {
        DocExpr!();
        DocAtom!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl DocExpr { fn parse < S : Copy > (tt : & tt :: TopSubtree < S >) -> DocExpr { next_doc_expr (tt . iter ()) . unwrap_or (DocExpr :: Invalid) } pub fn aliases (& self) -> & [Symbol] { match self { DocExpr :: Atom (DocAtom :: KeyValue { key , value }) if * key == sym :: alias => { std :: slice :: from_ref (value) } DocExpr :: Alias (aliases) => aliases , _ => & [] , } } }
    };
}

impl_22!()
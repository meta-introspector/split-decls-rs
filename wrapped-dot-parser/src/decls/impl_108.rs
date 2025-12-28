macro_rules! deps {
    () => {
        EdgeStmt!();
        AList!();
        EdgeSet!();
        Edge!();
        NodeSet!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < A > From < (EdgeStmt < A > , & mut NodeSet < A >) > for EdgeSet < A > where A : Clone , { fn from (tuple : (EdgeStmt < A > , & mut NodeSet < A >)) -> Self { let (stmt , nodes) = tuple ; let mut from = stmt . from ; let mut rhs = stmt . next ; let mut set = Vec :: new () ; let attr = stmt . attr . map (| list | list . into ()) . unwrap_or (AList :: empty ()) ; loop { let to = rhs . to ; let from_id = from . id . clone () ; let to_id = to . id . clone () ; nodes . insert_if_absent (from . id . to_string () , (& from) . into ()) ; nodes . insert_if_absent (to . id . to_string () , (& to) . into ()) ; let edge = Edge { from : from_id . to_string () , to : to_id . to_string () , attr : attr . clone () , } ; set . push (edge) ; if rhs . next . is_none () { return EdgeSet { set } ; } from = to ; rhs = * (rhs . next . unwrap ()) ; } } }
    };
}

impl_108!()
macro_rules! deps {
    () => {
        VerifyBound!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < 'tcx > VerifyBound < 'tcx > { pub fn must_hold (& self) -> bool { match self { VerifyBound :: IfEq (..) => false , VerifyBound :: OutlivedBy (re) => re . is_static () , VerifyBound :: IsEmpty => false , VerifyBound :: AnyBound (bs) => bs . iter () . any (| b | b . must_hold ()) , VerifyBound :: AllBounds (bs) => bs . iter () . all (| b | b . must_hold ()) , } } pub fn cannot_hold (& self) -> bool { match self { VerifyBound :: IfEq (..) => false , VerifyBound :: IsEmpty => false , VerifyBound :: OutlivedBy (_) => false , VerifyBound :: AnyBound (bs) => bs . iter () . all (| b | b . cannot_hold ()) , VerifyBound :: AllBounds (bs) => bs . iter () . any (| b | b . cannot_hold ()) , } } pub fn or (self , vb : VerifyBound < 'tcx >) -> VerifyBound < 'tcx > { if self . must_hold () || vb . cannot_hold () { self } else if self . cannot_hold () || vb . must_hold () { vb } else { VerifyBound :: AnyBound (vec ! [self , vb]) } } }
    };
}

impl_145!()
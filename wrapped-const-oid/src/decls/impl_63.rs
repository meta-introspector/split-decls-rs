macro_rules! deps {
    () => {
        ObjectIdentifier!();
        Error!();
        Result!();
        Arc!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        # [cfg (feature = "arbitrary")] impl < 'a > arbitrary :: Arbitrary < 'a > for ObjectIdentifier { fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Self > { let first = u . int_in_range (0 ..= arcs :: ARC_MAX_FIRST) ? ; let second = u . int_in_range (0 ..= arcs :: ARC_MAX_SECOND) ? ; let third = u . arbitrary () ? ; let mut oid = Self :: from_arcs ([first , second , third]) . map_err (| _ | arbitrary :: Error :: IncorrectFormat) ? ; for arc in u . arbitrary_iter () ? { oid = oid . push_arc (arc ?) . map_err (| _ | arbitrary :: Error :: IncorrectFormat) ? ; } Ok (oid) } fn size_hint (depth : usize) -> (usize , Option < usize >) { (Arc :: size_hint (depth) . 0 . saturating_mul (3) , None) } }
    };
}

impl_63!();
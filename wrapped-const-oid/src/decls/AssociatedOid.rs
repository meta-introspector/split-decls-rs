macro_rules! deps {
    () => {
        ObjectIdentifier!();
    };
}

macro_rules! AssociatedOid {
    () => {
        deps!();
        # [doc = " A trait which associates an OID with a type."] pub trait AssociatedOid { # [doc = " The OID associated with this type."] const OID : ObjectIdentifier ; }
    };
}

AssociatedOid!()
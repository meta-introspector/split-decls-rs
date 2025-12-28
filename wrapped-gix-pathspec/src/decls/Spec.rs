macro_rules! deps {
    () => {
        Pattern!();
    };
}

macro_rules! Spec {
    () => {
        deps!();
        # [derive (Default , Clone , Debug)] pub (crate) struct Spec { pub pattern : Pattern , pub attrs_match : Option < gix_attributes :: search :: Outcome > , }
    };
}

Spec!();
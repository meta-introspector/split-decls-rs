macro_rules! deps {
    () => {
        Integer!();
        Limb!();
    };
}

macro_rules! as_limb {
    () => {
        deps!();
        # [doc = " Cast to limb type."] # [inline] pub (crate) fn as_limb < T : Integer > (t : T) -> Limb { Limb :: as_cast (t) }
    };
}

as_limb!();
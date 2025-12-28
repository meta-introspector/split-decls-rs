macro_rules! deps {
    () => {
        Limb!();
        ReverseView!();
    };
}

macro_rules! rview {
    () => {
        deps!();
        # [doc = " Create a reverse view of the vector for indexing."] # [inline] pub fn rview (x : & [Limb]) -> ReverseView < Limb > { ReverseView { inner : x , } }
    };
}

rview!();
macro_rules! Map {
    () => {
        # [doc = " An iterator which applies a transform to elements."] pub struct Map < I , F , B > { it : I , f : F , value : Option < B > , }
    };
}

Map!();
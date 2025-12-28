macro_rules! deps {
    () => {
        Tree!();
        Clone!();
    };
}

macro_rules! BreadthFirstPresets {
    () => {
        deps!();
        # [doc = " Presets for common choices in breadth-first traversal."] # [derive (Copy , Clone)] pub struct BreadthFirstPresets < 'a , 'repo > { root : & 'a Tree < 'repo > , }
    };
}

BreadthFirstPresets!()
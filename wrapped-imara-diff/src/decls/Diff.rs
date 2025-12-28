macro_rules! Diff {
    () => {
        # [derive (Default)] pub struct Diff { removed : Vec < bool > , added : Vec < bool > , }
    };
}

Diff!()
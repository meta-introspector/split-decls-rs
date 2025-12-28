macro_rules! deps {
    () => {
        TreeInfoTuple!();
        ChangeId!();
    };
}

macro_rules! State {
    () => {
        deps!();
        # [doc = " The state required to run [tree-diffs](super::tree())."] # [derive (Default , Clone)] pub struct State { # [doc = " A buffer for object data."] pub buf1 : Vec < u8 > , # [doc = " Another buffer for object data."] pub buf2 : Vec < u8 > , trees : VecDeque < TreeInfoTuple > , change_id : visit :: ChangeId , }
    };
}

State!()
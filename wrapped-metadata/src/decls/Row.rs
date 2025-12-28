macro_rules! deps {
    () => {
        TypeIndex!();
    };
}

macro_rules! Row {
    () => {
        deps!();
        # [derive (Copy , Clone)] pub struct Row < 'a > { pub index : & 'a TypeIndex , pub file : usize , pub pos : usize , }
    };
}

Row!();
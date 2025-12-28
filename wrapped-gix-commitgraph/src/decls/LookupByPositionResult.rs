macro_rules! deps {
    () => {
        File!();
        Position!();
    };
}

macro_rules! LookupByPositionResult {
    () => {
        deps!();
        # [derive (Clone)] struct LookupByPositionResult < 'a > { pub file : & 'a File , pub _file_index : usize , pub pos : file :: Position , }
    };
}

LookupByPositionResult!();
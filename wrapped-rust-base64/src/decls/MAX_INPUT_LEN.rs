macro_rules! MAX_INPUT_LEN {
    () => {
        # [doc = " The most bytes whose encoding will fit in `BUF_SIZE`"] const MAX_INPUT_LEN : usize = BUF_SIZE / 4 * 3 ;
    };
}

MAX_INPUT_LEN!()
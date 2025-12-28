macro_rules! Matrix {
    () => {
        # [derive (Clone)] pub struct Matrix { bytes : Vec < u8 > , ncols : usize , nrows : usize , }
    };
}

Matrix!()
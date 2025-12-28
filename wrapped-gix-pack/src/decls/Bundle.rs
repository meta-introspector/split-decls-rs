macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! Bundle {
    () => {
        deps!();
        # [doc = " A bundle of pack data and the corresponding pack index"] pub struct Bundle { # [doc = " The pack file corresponding to `index`"] pub pack : data :: File , # [doc = " The index file corresponding to `pack`"] pub index : index :: File , }
    };
}

Bundle!();
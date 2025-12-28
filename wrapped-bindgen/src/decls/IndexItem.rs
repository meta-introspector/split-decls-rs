macro_rules! IndexItem {
    () => {
        # [derive (Default , Serialize)] struct IndexItem { # [serde (rename = "n")] name : String , # [serde (rename = "f")] features : Vec < usize > , }
    };
}

IndexItem!()
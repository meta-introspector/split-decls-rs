macro_rules! get_required_uses_for_item_const {
    () => {
        pub fn get_required_uses_for_item_const (_constant : & syn :: ItemConst) -> String { "" . to_string () }
    };
}

get_required_uses_for_item_const!()
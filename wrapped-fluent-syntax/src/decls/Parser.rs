macro_rules! Parser {
    () => {
        pub struct Parser < S > { pub (super) source : S , pub (super) ptr : usize , pub (super) length : usize , }
    };
}

Parser!();
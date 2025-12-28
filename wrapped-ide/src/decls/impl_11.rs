macro_rules! deps {
    () => {
        Markup!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Markup { pub fn as_str (& self) -> & str { self . text . as_str () } pub fn fenced_block (contents : impl fmt :: Display) -> Markup { format ! ("```rust\n{contents}\n```") . into () } pub fn fenced_block_text (contents : impl fmt :: Display) -> Markup { format ! ("```text\n{contents}\n```") . into () } }
    };
}

impl_11!()
macro_rules! M128A {
    () => {
        # [repr (C)] # [derive (Clone , Copy)] pub struct M128A { pub Low : u64 , pub High : i64 , }
    };
}

M128A!()
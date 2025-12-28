macro_rules! deps {
    () => {
        Decision!();
    };
}

macro_rules! test_filter {
    () => {
        deps!();
        # [cfg (test)] # [allow (unused_variables)] fn test_filter (level : u32 , key : & [u8] , value : & [u8]) -> Decision { use self :: Decision :: { Change , Keep , Remove } ; match key . first () { Some (& b'_') => Remove , Some (& b'%') => Change (b"secret") , _ => Keep , } }
    };
}

test_filter!()
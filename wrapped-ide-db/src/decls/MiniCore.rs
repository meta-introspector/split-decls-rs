macro_rules! MiniCore {
    () => {
        # [derive (Debug , Clone , Copy)] pub struct MiniCore < 'a > (& 'a str) ;
    };
}

MiniCore!();
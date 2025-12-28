macro_rules! DebugPrint {
    () => {
        # [doc (hidden)] pub struct DebugPrint < 'a , T : Debug > (pub & 'a T) ;
    };
}

DebugPrint!();
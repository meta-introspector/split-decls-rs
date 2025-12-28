macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! STV_PROTECTED {
    () => {
        deps!();
        # [doc = " Symbol is visible to other components, but is not preemptible."] pub const STV_PROTECTED : u8 = 3 ;
    };
}

STV_PROTECTED!();
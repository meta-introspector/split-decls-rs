macro_rules! deps {
    () => {
        Stack!();
    };
}

macro_rules! Platform {
    () => {
        deps!();
        # [must_use] pub struct Platform < 'a > { parent : & 'a Stack , is_dir : Option < bool > , }
    };
}

Platform!();
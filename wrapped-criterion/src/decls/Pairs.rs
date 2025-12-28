macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! Pairs {
    () => {
        deps!();
        # [doc = " Iterator over `Data`"] pub struct Pairs < 'a , X : 'a , Y : 'a > { data : Data < 'a , X , Y > , state : usize , }
    };
}

Pairs!();
macro_rules! deps {
    () => {
        Display!();
    };
}

macro_rules! Trait {
    () => {
        deps!();
        # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Debug)] pub enum Trait { Debug , Display , Octal , LowerHex , UpperHex , Pointer , Binary , LowerExp , UpperExp , }
    };
}

Trait!()
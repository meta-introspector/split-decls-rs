macro_rules! Scan {
    () => {
        # [derive (Copy , Clone , PartialEq)] enum Scan { Fail , Bailout , Consume , }
    };
}

Scan!();
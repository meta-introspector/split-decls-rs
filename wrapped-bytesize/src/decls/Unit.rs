macro_rules! Unit {
    () => {
        enum Unit { Byte , KiloByte , MegaByte , GigaByte , TeraByte , PetaByte , ExaByte , KibiByte , MebiByte , GibiByte , TebiByte , PebiByte , ExbiByte , }
    };
}

Unit!();
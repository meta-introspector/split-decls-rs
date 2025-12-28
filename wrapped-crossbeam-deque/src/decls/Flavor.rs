macro_rules! deps {
    () => {
        Worker!();
    };
}

macro_rules! Flavor {
    () => {
        deps!();
        # [doc = " Worker queue flavor: FIFO or LIFO."] # [derive (Clone , Copy , Debug , Eq , PartialEq)] enum Flavor { # [doc = " The first-in first-out flavor."] Fifo , # [doc = " The last-in first-out flavor."] Lifo , }
    };
}

Flavor!()
macro_rules! deps {
    () => {
        Counter!();
        Instructions!();
    };
}

macro_rules! InstructionsMinusIrqs {
    () => {
        deps!();
        # [doc = " More accurate [`Instructions`] (subtracting hardware interrupt counts)."] # [doc = ""] # [doc = " Can be obtained with `Counter::by_name(\"instructions-minus-irqs:u\")`."] pub struct InstructionsMinusIrqs { instructions : hw :: Counter , irqs : hw :: Counter , start : u64 , }
    };
}

InstructionsMinusIrqs!()
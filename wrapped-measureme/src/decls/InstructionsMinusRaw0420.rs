macro_rules! deps {
    () => {
        InstructionsMinusIrqs!();
    };
}

macro_rules! InstructionsMinusRaw0420 {
    () => {
        deps!();
        # [doc = " (Experimental) Like [`InstructionsMinusIrqs`] (but using an undocumented `r0420:u` counter)."] # [doc = ""] # [doc = " Can be obtained with `Counter::by_name(\"instructions-minus-r0420:u\")`."] pub struct InstructionsMinusRaw0420 (InstructionsMinusIrqs) ;
    };
}

InstructionsMinusRaw0420!();
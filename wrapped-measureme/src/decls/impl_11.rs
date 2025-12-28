macro_rules! deps {
    () => {
        HwCounterType!();
        Instructions!();
        InstructionsMinusIrqs!();
        InstructionsMinusRaw0420!();
        Counter!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl InstructionsMinusRaw0420 { const NAME : & 'static str = "instructions-minus-r0420:u" ; pub fn new () -> Result < Self , Box < dyn Error + Send + Sync > > { let model = hw :: CpuModel :: detect () ? ; let instructions = hw :: Counter :: new (& model , HwCounterType :: Instructions) ? ; let irqs = hw :: Counter :: new (& model , HwCounterType :: Raw0420) ? ; let (start_instructions , start_irqs) = (& instructions , & irqs) . read () ; let start = start_instructions . wrapping_sub (start_irqs) ; Ok (InstructionsMinusRaw0420 (InstructionsMinusIrqs { instructions , irqs , start , })) } # [inline] fn since_start (& self) -> u64 { self . 0 . since_start () } }
    };
}

impl_11!()
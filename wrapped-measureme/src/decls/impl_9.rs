macro_rules! deps {
    () => {
        InstructionsMinusIrqs!();
        Counter!();
        HwCounterType!();
        Instructions!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl InstructionsMinusIrqs { const NAME : & 'static str = "instructions-minus-irqs:u" ; pub fn new () -> Result < Self , Box < dyn Error + Send + Sync > > { let model = hw :: CpuModel :: detect () ? ; let instructions = hw :: Counter :: new (& model , HwCounterType :: Instructions) ? ; let irqs = hw :: Counter :: new (& model , HwCounterType :: Irqs) ? ; let (start_instructions , start_irqs) = (& instructions , & irqs) . read () ; let start = start_instructions . wrapping_sub (start_irqs) ; Ok (InstructionsMinusIrqs { instructions , irqs , start , }) } # [inline] fn since_start (& self) -> u64 { let (instructions , irqs) = (& self . instructions , & self . irqs) . read () ; instructions . wrapping_sub (irqs) . wrapping_sub (self . start) } }
    };
}

impl_9!();
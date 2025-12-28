macro_rules! deps {
    () => {
        OperationIter!();
        Expression!();
        Encoding!();
        Reader!();
        Evaluation!();
    };
}

macro_rules! impl_516 {
    () => {
        deps!();
        impl < R : Reader > Expression < R > { # [doc = " Create an evaluation for this expression."] # [doc = ""] # [doc = " The `encoding` is determined by the"] # [doc = " [`CompilationUnitHeader`](struct.CompilationUnitHeader.html) or"] # [doc = " [`TypeUnitHeader`](struct.TypeUnitHeader.html) that this expression"] # [doc = " relates to."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```rust,no_run"] # [doc = " use gimli::Expression;"] # [doc = " # let endian = gimli::LittleEndian;"] # [doc = " # let debug_info = gimli::DebugInfo::from(gimli::EndianSlice::new(&[], endian));"] # [doc = " # let unit = debug_info.units().next().unwrap().unwrap();"] # [doc = " # let bytecode = gimli::EndianSlice::new(&[], endian);"] # [doc = " let expression = gimli::Expression(bytecode);"] # [doc = " let mut eval = expression.evaluation(unit.encoding());"] # [doc = " let mut result = eval.evaluate().unwrap();"] # [doc = " ```"] # [cfg (feature = "read")] # [inline] pub fn evaluation (self , encoding : Encoding) -> Evaluation < R > { Evaluation :: new (self . 0 , encoding) } # [doc = " Return an iterator for the operations in the expression."] pub fn operations (self , encoding : Encoding) -> OperationIter < R > { OperationIter { input : self . 0 , encoding , } } }
    };
}

impl_516!();
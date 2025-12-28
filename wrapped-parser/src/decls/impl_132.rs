macro_rules! deps {
    () => {
        Pos!();
        PositionCalculator!();
    };
}

macro_rules! impl_132 {
    () => {
        deps!();
        impl < 'a > PositionCalculator < 'a > { pub (crate) fn new (input : & 'a str) -> PositionCalculator < 'a > { Self { input , pos : 0 , line : 1 , column : 1 , } } pub (crate) fn step < R : RuleType > (& mut self , pair : & Pair < R >) -> Pos { let pos = pair . as_span () . start () ; debug_assert ! (pos >= self . pos) ; let bytes_to_read = pos - self . pos ; let chars_to_read = self . input [.. bytes_to_read] . chars () ; for ch in chars_to_read { match ch { '\r' => { self . column = 1 ; } '\n' => { self . line += 1 ; self . column = 1 ; } _ => { self . column += 1 ; } } } self . pos = pos ; self . input = & self . input [bytes_to_read ..] ; Pos { line : self . line , column : self . column , } } }
    };
}

impl_132!()
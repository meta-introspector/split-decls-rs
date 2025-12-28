macro_rules! deps {
    () => {
        Assoc!();
        PrecClimber!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        # [cfg (feature = "const_prec_climber")] impl < R : Clone + 'static > PrecClimber < R > { # [doc = " Creates a new `PrecClimber` directly from a static slice of"] # [doc = " `(rule: Rule, precedence: u32, associativity: Assoc)` tuples."] # [doc = ""] # [doc = " Precedence starts from `1`.  Entries don't have to be ordered in any way, but it's easier to read when"] # [doc = " sorted."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use pest::prec_climber::{Assoc, PrecClimber};"] # [doc = " # #[allow(non_camel_case_types)]"] # [doc = " # #[allow(dead_code)]"] # [doc = " # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]"] # [doc = " # enum Rule {"] # [doc = " #     plus,"] # [doc = " #     minus,"] # [doc = " #     times,"] # [doc = " #     divide,"] # [doc = " #     power"] # [doc = " # }"] # [doc = " static CLIMBER: PrecClimber<Rule> = PrecClimber::new_const(&["] # [doc = "     (Rule::plus, 1, Assoc::Left), (Rule::minus, 1, Assoc::Left),"] # [doc = "     (Rule::times, 2, Assoc::Left), (Rule::divide, 2, Assoc::Left),"] # [doc = "     (Rule::power, 3, Assoc::Right)"] # [doc = " ]);"] # [doc = " ```"] pub const fn new_const (ops : & 'static [(R , u32 , Assoc)]) -> PrecClimber < R > { PrecClimber { ops : Cow :: Borrowed (ops) , } } }
    };
}

impl_148!();
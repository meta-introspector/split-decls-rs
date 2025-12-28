macro_rules! Precedence {
    () => {
        # [derive (Copy , Clone , PartialEq , PartialOrd)] pub enum Precedence { Jump , Assign , Range , Or , And , Let , Compare , BitOr , BitXor , BitAnd , Shift , Sum , Product , Cast , Prefix , Unambiguous , }
    };
}

Precedence!();
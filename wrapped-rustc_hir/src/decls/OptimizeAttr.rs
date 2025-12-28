macro_rules! deps {
    () => {
        PrintAttribute!();
    };
}

macro_rules! OptimizeAttr {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , PartialEq , Eq , Default , PrintAttribute)] # [derive (Encodable , Decodable , HashStable_Generic)] pub enum OptimizeAttr { # [doc = " No `#[optimize(..)]` attribute"] # [default] Default , # [doc = " `#[optimize(none)]`"] DoNotOptimize , # [doc = " `#[optimize(speed)]`"] Speed , # [doc = " `#[optimize(size)]`"] Size , }
    };
}

OptimizeAttr!();
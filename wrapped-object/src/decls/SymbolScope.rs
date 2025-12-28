macro_rules! deps {
    () => {
        Dynamic!();
        Symbol!();
    };
}

macro_rules! SymbolScope {
    () => {
        deps!();
        # [doc = " A symbol scope."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum SymbolScope { # [doc = " Unknown scope."] Unknown , # [doc = " Symbol is visible to the compilation unit."] Compilation , # [doc = " Symbol is visible to the static linkage unit."] Linkage , # [doc = " Symbol is visible to dynamically linked objects."] Dynamic , }
    };
}

SymbolScope!();
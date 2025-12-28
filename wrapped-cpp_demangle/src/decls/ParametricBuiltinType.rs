macro_rules! deps {
    () => {
        Number!();
        Expression!();
    };
}

macro_rules! ParametricBuiltinType {
    () => {
        deps!();
        # [doc = " <builtin-type> ::= DF <number> _ # ISO/IEC TS 18661 binary floating point type _FloatN (N bits), C++23 std::floatN_t"] # [doc = "                ::= DF <number> x # IEEE extended precision formats, C23 _FloatNx (N bits)"] # [doc = "                ::= DB <number> _        # C23 signed _BitInt(N)"] # [doc = "                ::= DB <instantiation-dependent expression> _ # C23 signed _BitInt(N)"] # [doc = "                ::= DU <number> _        # C23 unsigned _BitInt(N)"] # [doc = "                ::= DU <instantiation-dependent expression> _ # C23 unsigned _BitInt(N)"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum ParametricBuiltinType { # [doc = " _FloatN"] FloatN (Number) , # [doc = " _FloatNx"] FloatNx (Number) , # [doc = " signed _BitInt(N)"] SignedBitInt (Number) , # [doc = " unsigned _BitInt(N)"] UnsignedBitInt (Number) , # [doc = " signed _BitInt(expr)"] SignedBitIntExpression (Box < Expression >) , # [doc = " unsigned _BitInt(expr)"] UnsignedBitIntExpression (Box < Expression >) , }
    };
}

ParametricBuiltinType!()
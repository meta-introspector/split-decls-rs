macro_rules! deps {
    () => {
        Idx!();
        IndexVec!();
        IndexSlice!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl < I : Idx , J : Idx > IndexSlice < I , J > { # [doc = " Invert a bijective mapping, i.e. `invert(map)[y] = x` if `map[x] = y`,"] # [doc = " assuming the values in `self` are a permutation of `0..self.len()`."] # [doc = ""] # [doc = " This is used to go between `memory_index` (source field order to memory order)"] # [doc = " and `inverse_memory_index` (memory order to source field order)."] # [doc = " See also `FieldsShape::Arbitrary::memory_index` for more details."] pub fn invert_bijective_mapping (& self) -> IndexVec < J , I > { debug_assert_eq ! (self . iter () . map (| x | x . index () as u128) . sum ::< u128 > () , (0 .. self . len () as u128) . sum ::< u128 > () , "The values aren't 0..N in input {self:?}" ,) ; let mut inverse = IndexVec :: from_elem_n (Idx :: new (0) , self . len ()) ; for (i1 , & i2) in self . iter_enumerated () { inverse [i2] = i1 ; } debug_assert_eq ! (inverse . iter () . map (| x | x . index () as u128) . sum ::< u128 > () , (0 .. inverse . len () as u128) . sum ::< u128 > () , "The values aren't 0..N in result {self:?}" ,) ; inverse } }
    };
}

impl_96!();
macro_rules! deps {
    () => {
        InPlaceSeed!();
        Vec!();
        Allocator!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de , T , A > serde :: de :: Deserialize < 'de > for Vec < T , A > where T : serde :: de :: Deserialize < 'de > , A : Allocator + Default , { # [inline (always)] fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct VecVisitor < T , A > { marker : PhantomData < (T , A) > , } impl < 'de , T , A > serde :: de :: Visitor < 'de > for VecVisitor < T , A > where T : serde :: de :: Deserialize < 'de > , A : Allocator + Default , { type Value = Vec < T , A > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < S > (self , mut seq : S) -> Result < Self :: Value , S :: Error > where S : serde :: de :: SeqAccess < 'de > , { let mut values = Vec :: with_capacity_in (cautious (seq . size_hint ()) , A :: default ()) ; while let Some (value) = seq . next_element () ? { values . push (value) ; } Ok (values) } } let visitor = VecVisitor { marker : PhantomData , } ; deserializer . deserialize_seq (visitor) } # [inline (always)] fn deserialize_in_place < D > (deserializer : D , place : & mut Self) -> Result < () , D :: Error > where D : serde :: de :: Deserializer < 'de > , { struct VecInPlaceVisitor < 'a , T : 'a , A : Allocator + 'a > (& 'a mut Vec < T , A >) ; impl < 'a , 'de , T , A > serde :: de :: Visitor < 'de > for VecInPlaceVisitor < 'a , T , A > where T : serde :: de :: Deserialize < 'de > , A : Allocator + Default , { type Value = () ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a sequence") } fn visit_seq < S > (self , mut seq : S) -> Result < Self :: Value , S :: Error > where S : serde :: de :: SeqAccess < 'de > , { let hint = cautious (seq . size_hint ()) ; if let Some (additional) = hint . checked_sub (self . 0 . len ()) { self . 0 . reserve (additional) ; } for i in 0 .. self . 0 . len () { let next = { let next_place = InPlaceSeed (& mut self . 0 [i]) ; seq . next_element_seed (next_place) ? } ; if next . is_none () { self . 0 . truncate (i) ; return Ok (()) ; } } while let Some (value) = seq . next_element () ? { self . 0 . push (value) ; } Ok (()) } } deserializer . deserialize_seq (VecInPlaceVisitor (place)) } }
    };
}

impl_194!();
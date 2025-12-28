macro_rules! InstructionSetAttr {
    () => {
        # [derive (Clone , Encodable , Decodable , Debug , PartialEq , Eq , HashStable_Generic)] pub enum InstructionSetAttr { ArmA32 , ArmT32 , }
    };
}

InstructionSetAttr!();
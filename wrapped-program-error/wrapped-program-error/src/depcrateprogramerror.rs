// Generated macro for ProgramError (enum)
macro_rules! DepcrateProgramError {
() => {
// Module: crate
// Provides: {"ProgramError"}
// Dependencies: {}
# [doc = " Reasons the program may fail"] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Clone , Debug , Eq , PartialEq)] pub enum ProgramError { # [doc = " Allows on-chain programs to implement program-specific error types and see them returned"] # [doc = " by the Solana runtime. A program-specific error may be any type that is represented as"] # [doc = " or serialized to a u32 integer."] Custom (u32) , InvalidArgument , InvalidInstructionData , InvalidAccountData , AccountDataTooSmall , InsufficientFunds , IncorrectProgramId , MissingRequiredSignature , AccountAlreadyInitialized , UninitializedAccount , NotEnoughAccountKeys , AccountBorrowFailed , MaxSeedLengthExceeded , InvalidSeeds , BorshIoError , AccountNotRentExempt , UnsupportedSysvar , IllegalOwner , MaxAccountsDataAllocationsExceeded , InvalidRealloc , MaxInstructionTraceLengthExceeded , BuiltinProgramsMustConsumeComputeUnits , InvalidAccountOwner , ArithmeticOverflow , Immutable , IncorrectAuthority , }
};
}

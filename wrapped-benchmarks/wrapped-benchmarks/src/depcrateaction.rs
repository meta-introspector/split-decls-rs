// Generated macro for Action (enum)
macro_rules! DepcrateAction {
() => {
// Module: crate
// Provides: {"Action"}
// Dependencies: {}
# [derive (BorshSerialize , BorshDeserialize , Debug , Clone , Eq , PartialEq , SerdeSerialize , SerdeDeserialize , Readable , Writable ,)] pub enum Action { CreateAccount (CreateAccountAction) , DeployContract (DeployContractAction) , FunctionCall (FunctionCallAction) , Transfer (TransferAction) , Stake (StakeAction) , AddKey (AddKeyAction) , DeleteKey (DeleteKeyAction) , DeleteAccount (DeleteAccountAction) , }
};
}

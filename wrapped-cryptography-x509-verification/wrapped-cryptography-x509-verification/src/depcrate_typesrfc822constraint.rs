// Generated macro for RFC822Constraint (enum)
macro_rules! Depcrate_typesRFC822Constraint {
() => {
// Module: crate::types
// Provides: {"RFC822Constraint"}
// Dependencies: {}
# [doc = " An `RFC822Constraint` represents a Name Constraint on email addresses."] pub enum RFC822Constraint < 'a > { # [doc = " A constraint for an exact match on a specific email address."] Exact (RFC822Name < 'a >) , # [doc = " A constraint for any mailbox on a particular domain."] OnDomain (DNSName < 'a >) , # [doc = " A constraint for any mailbox *within* a particular domain."] # [doc = " For example, `InDomain(\"example.com\")` will match `foo@bar.example.com`"] # [doc = " but not `foo@example.com`, since `bar.example.com` is in `example.com`"] # [doc = " but `example.com` is not within itself."] InDomain (DNSName < 'a >) , }
};
}
